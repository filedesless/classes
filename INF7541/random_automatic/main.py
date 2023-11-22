import random
import functools
import string
import graphviz

K = 2
STATE_NUMBER = random.randint(5, 10)
OUTPUT_LEN = 100


def main():
    dfa = DFA(K, STATE_NUMBER)

    output = ""
    for i in range(OUTPUT_LEN):
        term = dfa.read_int(i)
        print(f'{i} -> {term} ({string.ascii_letters[term]})')
        output += string.ascii_letters[term]
    print(output)
    dfa.view()


class DFA:
    def __init__(self, k: int, n: int):
        self.states = range(n)
        self.input_alphabet = range(k)
        self.initial_state = random.choice(self.states)
        self.output_alphabet = range(n)
        self.transition = {(state, symbol): random.choice(self.states)
                           for state in self.states for symbol in self.input_alphabet}
        self.output = {state: random.choice(self.output_alphabet) for state in self.states}
        self.desc = f'random automata with {n} states and alphabet of size {k} starting on {self.initial_state}'
        print(self.desc)

    def extended_transition(self, state: int, symbols: list[int]) -> int:
        for symbol in symbols:
            state = self.transition[(state, symbol)]
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
        output = self.output[state]
        print('which maps to output', output)
        return output

    def view(self):
        dot = graphviz.Digraph()
        for state in self.states:
            dot.node(str(state), f"{state} | {self.output[state]}")
            for symbol in self.input_alphabet:
                dst = self.transition[(state, symbol)]
                dot.edge(str(state), str(dst), str(symbol))
        dot.node('start')
        dot.edge('start', str(self.initial_state))
        dot.view()


if __name__ == '__main__':
    main()
