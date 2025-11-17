#!/usr/bin/env python3

import argparse
import chess
import chess.pgn
import sys
import traceback
import io

from subprocess import Popen, PIPE
from batched_execution_pool import BatchedExecutionPool

class Engine():

    def __init__(self, binary):
        self.engine = Popen([binary], stdin=PIPE, stdout=PIPE, universal_newlines=True, shell=True)
        self.uci_ready()

    def write_line(self, line):
        self.engine.stdin.write(line)
        self.engine.stdin.flush()

    def read_line(self):
        return self.engine.stdout.readline().rstrip()

    def uci_ready(self):
        self.write_line('isready\n')
        while self.read_line() != 'readyok': pass

    def quit(self):
        self.write_line('quit\n')

def parse_args():

    p = argparse.ArgumentParser(description='Add UCI options with: --option.Name=Value')
    p.add_argument('--engine', type=str, required=True, help='Path to the engine or engine name')
    p.add_argument('--pgn',    type=str, required=True, help='Path to the PGN file')
    p.add_argument('--player', type=str, required=True, help='Name of the player')
    args, unknown = p.parse_known_args()

    uci_options = []
    for value in unknown:
        if '=' in value and value.startswith('--option.'):
            uci_options.append(value[len('--option.'):].split('='))

    return args, uci_options

def convert_uci_to_score(score_type, score_value):

    if score_type == 'cp' and int(score_value) == 0:
        return '0.00'

    if score_type == 'cp':
        return '%+.2f' % (float(score_value) / 100.0)

    if score_type == 'mate' and int(score_value) < 0:
        return '-M%d' % (abs(2 * int(score_value)))

    if score_type == 'mate' and int(score_value) > 0:
        return '+M%d' % (abs(2 * int(score_value) - 1))

    raise Exception('Unable to process Score (%s, %s)' % (score_type, score_value))


def game_generator(args):
    with open(args.pgn) as pgn_file:
        while game := chess.pgn.read_game(pgn_file):
            yield (str(game))

def replay_game(game_str, args):

    game = chess.pgn.read_game(io.StringIO(game_str))
    process_args, uci_options = args

    is_white = game.headers.get('White') == process_args.player
    is_black = game.headers.get('Black') == process_args.player
    assert is_white or is_black

    fen = game.headers.get('FEN', None)
    pos = 'position fen %s moves' % (fen) if fen else 'position startpos moves'

    engine = Engine(process_args.engine)
    for opt, value in uci_options:
        engine.write_line('setoption name %s value %s\n' % (opt, value))

    node = game
    while node.variations:

        next_node = node.variation(0)

        if (node.turn() and is_white) or (not node.turn() and is_black):

            try:
                pgn_score, pgn_depths, pgn_timems, pgn_nodes = next_node.comment.split()
                pgn_nodes = int(pgn_nodes)

            except Exception:
                break

            engine.uci_ready()
            engine.write_line('%s\ngo nodes %d\n' % (pos, pgn_nodes))

            score = None
            while 'bestmove' not in (line := engine.read_line()):
                if ' score ' in line:
                    score = line.split(' score ')[1].split('nodes ')[0].split()[:2]
            best_move = line.split()[1]

            try:
                assert best_move == next_node.move.uci()
                assert convert_uci_to_score(*score) == pgn_score
            except AssertionError:
                engine.quit()
                return 'Failed: ' + game_str

        pos += ' ' + next_node.move.uci()
        node = next_node

    engine.quit()

if __name__ == '__main__':

    args, uci_options = parse_args()

    pool = BatchedExecutionPool(
        input_generator       = game_generator(args),
        process_function      = replay_game,
        process_function_args = [args, uci_options],
    )

    for result in pool.execute(threads=15, batchsize=256):
        if result:
            print (result)
