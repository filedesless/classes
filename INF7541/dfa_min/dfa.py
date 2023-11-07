State = str
Symbol = str


class DFA:
    def __init__(self, Q: set[State], sigma: set[Symbol], delta: dict[(State, Symbol), State], q0: State, F: set[State]):
        self.Q = Q
        self.sigma = sigma
        self.delta = delta
        self.q0 = q0
        self.F = F

    def get_reachable_states(self) -> set[State]:
        reachable_states = {self.q0}
        new_states = {self.q0}

        while new_states:
            new_states = {self.delta[(q, c)] for q in new_states for c in self.sigma if (
                q, c) in self.delta} - reachable_states
            reachable_states |= new_states

        return reachable_states

    def get_alive_states(self) -> set[State]:
        alive_states = set(self.F)
        new_states = set(self.F)

        while new_states:
            new_states = {q for q in self.Q for c in self.sigma
                          if (q, c) in self.delta and self.delta[(q, c)] in alive_states} - alive_states
            alive_states |= new_states

        return alive_states

    def __repr__(self) -> str:
        return str(self.__dict__)


def minimize(dfa: DFA) -> DFA:
    # remove unreachable and dead states
    # dfa.Q = dfa.get_reachable_states() & dfa.get_alive_states()
    P = {frozenset(dfa.F), frozenset(dfa.Q - dfa.F)}
    W = {frozenset(dfa.F), frozenset(dfa.Q - dfa.F)}
    print('P', P, 'W', W)
    while W:
        A = W.pop()
        for c in dfa.sigma:
            X = {q for q in dfa.Q if dfa.delta.get((q, c)) in A}
            for Y in P.copy():
                a, m = frozenset(X & Y), frozenset(Y - X)
                if a and m:
                    P.remove(Y)
                    P.add(a)
                    P.add(m)
                    if Y in W:
                        W.remove(Y)
                        W.add(a)
                        W.add(m)
                    elif len(a) <= len(m):
                        W.add(a)
                    else:
                        W.add(m)

    print('P', P, 'W', W)
    Q = { ','.join(sorted(s)) for s in P }
    print('Q', Q)
    delta = {  (nq, c): nd
        for ((q, c), d) in dfa.delta.items()
        for nq in Q for nd in Q if q in nq and d in nd }
    
    dfa.Q = Q
    dfa.delta = delta
    dfa.q0 = next( q for q in dfa.Q if dfa.q0 in q )
    dfa.F = { q for f in dfa.F for q in dfa.Q if f in q }
    return dfa


# a initial, b final, c unreachable, d dead
# dfa = DFA({'a', 'b', 'c', 'd'}, {'0', '1'}, {('a', '1'): 'b', ('c', '1'): 'b', ('a', '0'): 'd'}, 'a', {'b'})
dfa = DFA(
    {'a', 'b', 'c', 'd', 'e', 'f'}, {'0', '1'}, {
        ('a', '0'): 'b',
        ('a', '1'): 'c',
        ('b', '0'): 'a',
        ('b', '1'): 'd',
        ('c', '0'): 'e',
        ('c', '1'): 'f',
        ('d', '0'): 'e',
        ('d', '1'): 'f',
        ('e', '0'): 'e',
        ('e', '1'): 'f',
        ('f', '0'): 'f',
        ('f', '1'): 'f',
    }, 'a', {'c', 'd', 'e'}
)
print('dfa', dfa)
print(minimize(dfa))
