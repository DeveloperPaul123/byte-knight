# originally from: https://gist.github.com/AndyGrant/a77e10c65f0059cce018e87f617a9446
import argparse
import chess
import chess.pgn


def iterate_uci_options(unknown):
    for arg in unknown:
        if "=" in arg and arg.startswith("--option."):
            yield arg[len("--option.") :].split("=")


def main():
    p = argparse.ArgumentParser(description="Add options with: --option.Name=Value")
    p.add_argument(
        "--pgn", type=str, required=True, help=".pgn with only a single game"
    )

    # Must pick between replicating game with fixed nodes or fixed depth
    p.add_argument(
        "--nodes", action="store_true", help='Generate commands using "go nodes"'
    )
    p.add_argument(
        "--depth", action="store_true", help='Generate commands using "go depth"'
    )

    # Provide commands only for the desired colour
    p.add_argument("--white", action="store_true", help="Generate commands for White")
    p.add_argument("--black", action="store_true", help="Generate commands for Black")

    args, unknown = p.parse_known_args()

    if args.nodes == args.depth:
        raise Exception("Must use either --nodes or --depth")

    if args.white == args.black:
        raise Exception("Must use either --white or --black")

    with open(args.pgn) as pgn_file:
        game = chess.pgn.read_game(pgn_file)

    if not game:
        raise Exception("Empty PGN file")

    print("uci")
    for opt_name, opt_value in iterate_uci_options(unknown):
        print("setoption name %s value %s" % (opt_name, opt_value))
    print("ucinewgame")
    print("isready")

    pos = "position startpos moves" if "FEN" not in game.headers else None
    pos = pos if pos else "position fen %s moves" % (game.headers["FEN"])

    node = game
    while node.variations:
        next_node = node.variation(0)
        move = next_node.move.uci()
        comment = next_node.comment

        if (node.turn() and args.white) or (not node.turn() and args.black):
            print(pos)

            try:
                score, depths, timems, nodes = comment.split()
            except:
                exit()
            depth, seldepth = depths.split("/")

            print("go depth %s" % (depth) if args.depth else "go nodes %s" % (nodes))
            print("wait")

        pos += " " + move
        node = next_node


if __name__ == "__main__":
    main()
