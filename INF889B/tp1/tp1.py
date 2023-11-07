from pysat.solvers import Solver
from pysat.formula import IDPool
from enum import StrEnum, auto


class Personne(StrEnum):
    Britannique = auto()
    Allemand = auto()
    Norvegien = auto()
    Suedois = auto()
    Danois = auto()


class Animal(StrEnum):
    Chien = auto()
    Chat = auto()
    Oiseau = auto()
    Cheval = auto()
    Poisson = auto()


class Boisson(StrEnum):
    The = auto()
    Cafe = auto()
    Lait = auto()
    Eau = auto()
    Biere = auto()


class Cigare(StrEnum):
    PallMall = auto()
    Dunhill = auto()
    Prince = auto()
    Blend = auto()
    Bluemaster = auto()


class Maison(StrEnum):
    Rouge = auto()
    Verte = auto()
    Blanche = auto()
    Bleue = auto()
    Jaune = auto()


s = Solver('mc')
vpool = IDPool()
var = vpool.id

# Le Britannique vit dans la maison rouge.
s.add_clause([var((Personne.Britannique, Maison.Rouge))])

# Le Suédois a des chiens.
s.add_clause([var((Personne.Suedois, Animal.Chien))])

# Le Danois boit du thé.
s.add_clause([var((Personne.Danois, Boisson.The))])

# Le Norvégien habite dans la première maison en partant de la gauche.
s.add_clause([var((Personne.Norvegien, 1))])

# L’Allemand fume des Prince.
s.add_clause([var((Personne.Allemand, Cigare.Prince))])

# Chaque {animal, boisson, cigare, maison, position} est associée à une seule personne
for liste in [Animal, Boisson, Cigare, Maison, [1, 2, 3, 4, 5]]:
    for thing in liste:
        clause = [var((x, thing)) for x in Personne]
        s.add_clause(clause)
        s.add_atmost(clause, 1)

for x in Personne:
    # Chaque personne a exactement 1 {animal, boisson, cigare, maison, position}
    for liste in [Animal, Boisson, Cigare, Maison, [1, 2, 3, 4, 5]]:
        clause = [var((x, thing)) for thing in liste]
        # au moins 1 {animal, boisson, cigare, maison, position} par personne
        s.add_clause(clause)
        # au plus 1 {animal, boisson, cigare, maison, position} par personne
        s.add_atmost(clause, 1)

    # Le propriétaire de la maison verte boit du café.
    #     x verte => x cafe
    # <=> -(x verte) | x cafe -- def implication
    s.add_clause([-var((x, Maison.Verte)), var((x, Boisson.Cafe))])

    # La personne qui fume des Pall Mall élève des oiseaux.
    #     x pallmall => x oiseau
    # <=> -(x pallmall) | x oiseau -- def implication
    s.add_clause([-var((x, Cigare.PallMall)), var((x, Animal.Oiseau))])

    # Le propriétaire de la maison jaune fume des Dunhill.
    #     x jaune => x dunhill
    # <=> -(x jaune) | x dunhill -- def implication
    s.add_clause([-var((x, Maison.Jaune)), var((x, Cigare.Dunhill))])

    # La personne qui vit dans la maison du centre boit du lait.
    #     x 3 => x lait
    # <=> -(x 3) | x lait -- def implication
    s.add_clause([-var((x, 3)), var((x, Boisson.Lait))])

    # Celui qui fume des Bluemaster boit de la bière.
    #     x bluemaster => x biere
    # <=> -(x bluemaster) | x biere -- def implication
    s.add_clause([-var((x, Cigare.Bluemaster)), var((x, Boisson.Biere))])

    for i in range(1, 6):
        # Le Norvégien vit juste à côté de la maison bleue.
        #     x bleue & x i => Norvegien i-1 | Norvegien i+1
        # <=> -(x bleue & x i) | Norvegien i-1 | Norvegien i+1    -- def implication
        # <=> -(x bleue) | -(x i) | Norvegien i-1 | Norvegien i+1 -- De Morgan
        clause = [-var((x, Maison.Bleue)), -var((x, i))]
        if i < 5:
            clause.append(var((Personne.Norvegien, i+1)))
        if 1 < i:
            clause.append(var((Personne.Norvegien, i-1)))
        s.add_clause(clause)

        for y in Personne:
            # La maison verte est directement à gauche de la maison blanche.
            #     x verte & x i & y blanche => y i+1
            # <=> -(x verte & x i & y blanche) | y i+1       -- def implication
            # <=> -(x verte) | -(x i) | -(y blanche) | y i+i -- De Morgan
            clause = [-var((x, Maison.Verte)), -var((x, i)), -
                      var((y, Maison.Blanche))]
            if i < 5:
                clause.append(var((y, i+1)))
            s.add_clause(clause)

            # L’homme qui fume des Blend vit à côté de celui qui a des chats.
            #     x blend & x i & y chat => y i-1 | i i+1
            # <=> -(x blend & x i & y chat) | y i-1 | y i+1       -- def implication
            # <=> -(x blend) | -(x i) | -(y chat) | y i-1 | y i+1 -- De Morgan
            clause = [-var((x, Cigare.Blend)), -
                      var((x, i)), -var((y, Animal.Chat))]
            if i < 5:
                clause.append(var((y, i+1)))
            if 1 < i:
                clause.append(var((y, i-1)))
            s.add_clause(clause)

            # L’homme qui a un cheval est le voisin de celui qui fume des Dunhill.
            #     x cheval & x i & y dunhill => y i-1 | y i+1
            # <=> -(x cheval & x i & y dunhill) | y i-1 | y i+1       -- def implication
            # <=> -(x cheval) | -(x i) | -(y dunhill) | y i-1 | y i+1 -- De Morgan
            clause = [-var((x, Animal.Cheval)), -
                      var((x, i)), -var((y, Cigare.Dunhill))]
            if i < 5:
                clause.append(var((y, i+1)))
            if 1 < i:
                clause.append(var((y, i-1)))
            s.add_clause(clause)

            # L’homme qui fume des Blend a un voisin qui boit de l’eau
            #     x blend & x i & y eau => y i-1 | y i+1
            # <=> -(x blend & x i & y eau) => y i-1 | y i+1      -- def implication
            # <=> -(x blend) | -(x i) | -(y eau) | y i-1 | y i+1 -- De Morgan
            clause = [-var((x, Cigare.Blend)), -
                      var((x, i)), -var((y, Boisson.Eau))]
            if i < 5:
                clause.append(var((y, i+1)))
            if 1 < i:
                clause.append(var((y, i-1)))
            s.add_clause(clause)

s.solve()
sol = s.get_model()
# print(sol)
for x in Personne:
    print(x, ':', ", ".join(str(thing) for liste in [Animal, Boisson, Cigare, Maison, [1, 2, 3, 4, 5]]
                            for thing in liste
                            if sol[var((x, thing)) - 1] > 0))
