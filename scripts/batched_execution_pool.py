import multiprocessing
import typing


class BatchedExecutionPool:
    def __init__(
        self,
        input_generator: typing.Generator,
        process_function: typing.Callable[..., typing.Any],
        process_function_args: typing.Optional[typing.Iterable[typing.Any]],
    ) -> None:
        self.input_generator = input_generator
        self.process_function = process_function
        self.process_function_args = process_function_args

    def execute(self, threads: int, batchsize: int) -> int:
        in_queue = multiprocessing.Queue()
        out_queue = multiprocessing.Queue()

        workers = [
            multiprocessing.Process(
                target=BatchedExecutionPool._process_function_wrapper,
                args=(
                    in_queue,
                    out_queue,
                    self.process_function,
                    self.process_function_args,
                ),
                daemon=True,
            )
            for f in range(threads)
        ]

        for worker in workers:
            worker.start()

        while True:
            n = self._enqueue_elements(batchsize, in_queue)
            for f in range(n):
                yield out_queue.get()
            if n != batchsize:
                break

        for f in range(threads):
            in_queue.put(None)

        for worker in workers:
            worker.join()

    def _enqueue_elements(self, batchsize: int, in_queue: multiprocessing.Queue) -> int:
        for f in range(batchsize):
            try:
                in_queue.put(next(self.input_generator))
            except StopIteration:
                return f
        return batchsize

    @staticmethod
    def _process_function_wrapper(
        in_queue: multiprocessing.Queue,
        out_queue: multiprocessing.Queue,
        process_function: typing.Callable[..., typing.Any],
        process_function_args: typing.Optional[typing.Iterable[typing.Any]],
    ) -> None:
        while (data := in_queue.get()) != None:
            out_queue.put(process_function(data, process_function_args))
