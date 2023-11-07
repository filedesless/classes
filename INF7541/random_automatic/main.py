import random
import functools
import string

K = 2
STATE_NUMBER = random.randint(5, 10)
OUTPUT_LEN = 10

def main():
    dfa = DFA(K, STATE_NUMBER)

    output = ""
    for i in range(OUTPUT_LEN):
        term = dfa.read_int(i)
        print(f'{i} -> {term} ({string.ascii_letters[term]})')
        output += string.ascii_letters[term]
    print(output)


class DFA:
    def __init__(self, k: int, n: int):
        self.states = range(n)
        self.input_alphabet = range(k)
        self.initial_state = random.choice(self.states)
        self.output_alphabet = range(n)
        print(
            f'random automata with {n} states and alphabet of size {k} starting on {self.initial_state}')

    @functools.cache
    def transition(self, state: int, symbol: int) -> int:
        assert state in self.states
        assert symbol in self.input_alphabet
        return random.choice(self.states)

    def extended_transition(self, state: int, symbols: list[int]) -> int:
        for symbol in symbols:
            state = self.transition(state, symbol)
        return state

    def read_int(self, number: int) -> int:
        digits = []
        print(f'reading {number} in base {len(self.input_alphabet)}')
        while number:
            digits.append(int(number % len(self.input_alphabet)))
            number //= len(self.input_alphabet)
        digits.reverse()
        state = self.extended_transition(self.initial_state, digits)
        print(f'executing automaton on string {digits} leads to state {state}')
        output = self.output(state)
        print(f'which maps to output', output)
        return output

    @functools.cache
    def output(self, state: int) -> int:
        assert state in self.states
        return random.choice(self.output_alphabet)

if __name__ == '__main__':
    main()

