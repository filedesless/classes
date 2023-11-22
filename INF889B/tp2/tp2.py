"""
TP2: Randomized Traveling Salesman Problem approximation using Hill Climbing with restarts
Félix Larose-Gervais
"""
import itertools
import random
import collections


def random_cycle(length: int) -> list[int]:
    """ex: random_cycle(3) could be [1, 0, 2]"""
    return random.sample(range(length), k=length)


class TSP:
    """random Traveling Salesman Problem instance, generated atop a random k-cycle solution"""

    def __init__(self, n: int):
        self._solution: list[int] = random_cycle(n)
        self._cost: dict[(int, int), int] = collections.defaultdict(
            lambda: random.randint(2, 20))  # default small cost for solution-adjacent nodes
        print('TSP', self._solution, self.score_cycle(self._solution))

    def _cost_memo(self, a: int, b: int) -> int:
        assert a in self._solution and b in self._solution and a != b
        # sorted to have i < j
        i, j = sorted((self._solution.index(a), self._solution.index(b)))
        if j - i == 1 or (i == 0 and j == len(self._solution) - 1):  # adjacent
            # direct path in the solution with a random cost
            return self._cost[(a, b)]
        if (a, b) not in self._cost:
            # otherwise we can walk from a to b going left or right
            left = self._solution[:i+1][::-1] + self._solution[j:][::-1]
            right = self._solution[i:j+1]
            # add small epsilon
            self._cost[(a, b)] = random.randint(1, 10) + \
                min(self.score_path(left), self.score_path(right))
        return self._cost[(a, b)]

    def cost(self, a: int, b: int) -> int:
        """cost of going from a to b, assuming 
        - there's always a path
        - cost(a, b) == cost(b, a)"""
        return self._cost_memo(*sorted((a, b)))  # sorted out of respect for the cache

    def score_path(self, path: list[int]) -> int:
        """cost of following the whole path"""
        assert len(path) >= 2
        return sum(self.cost(x, y) for (x, y) in zip(path, path[1:]))

    def score_cycle(self, cycle: list[int]) -> int:
        """cost of following the whole path and going back to first node"""
        return self.score_path(cycle + [cycle[0]])

    def hill_climbing(self, n=1) -> list[int]:
        """finds an approximated solution by iterating 2-swaps from a random solution n times"""
        return min((self._hill_climbing(), i) for i in range(n))

    def _hill_climbing(self) -> list[int]:
        cycle: list[int] = random_cycle(len(self._solution))
        print('random initial solution', cycle, self.score_cycle(cycle))

        # Start climbing
        while True:
            best = self.score_cycle(cycle)
            for (i, j) in itertools.combinations(range(1, len(cycle) - 1), 2):
                cycle[i], cycle[j] = cycle[j], cycle[i]  # swap
                if self.score_cycle(cycle) < best:  # climb
                    break
                cycle[i], cycle[j] = cycle[j], cycle[i]  # undo
            else:  # didn't climb for any move, we reached local optimum
                break
        print('led to local optimum:', cycle, best)
        return (best, cycle)


if __name__ == "__main__":
    tsp = TSP(random.randint(20, 30))
    (score, sol), r = tsp.hill_climbing(10)
    print(f"found best {sol} with score {score} on run #{r}")
