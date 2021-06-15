import unittest
from sklearn.metrics.pairwise import cosine_similarity as sk_cosine_similarity


def cosine_similarity(vec1, vec2):
    """reference:
    https://clay-atlas.com/blog/2020/03/26/cosine-similarity-text-count/
    """
    dot = sum(x * y for x, y in zip(vec1, vec2))
    norm1 = sum(x ** 2 for x in vec1) ** 0.5
    norm2 = sum(y ** 2 for y in vec2) ** 0.5
    return dot / (norm1 * norm2)


class CosineSimilarityTest(unittest.TestCase):
    def test_cosine_similarity(self):
        v1 = [
            -0.046193234622478485,
            -0.09216824918985367,
            0.023753443732857704,
            -0.03982221707701683,
            0.030631808564066887,
            0.06340867280960083,
            -0.09439295530319214,
            -0.1576867550611496,
            0.459428995847702,
            -0.22166694700717926,
            0.21970123052597046,
            0.19883397221565247,
            -0.19289985299110413,
            -0.157765731215477,
            0.0013831154210492969,
            0.29028451442718506,
            0.18202221393585205,
            0.14411108195781708,
            0.43273560404777527,
            -0.31332970261573792,
        ]
        v2 = [
            0.23711472749710083,
            0.0747479647397995,
            0.20933881402015686,
            -0.1695360243320465,
            0.2809278070926666,
            0.2502232491970062,
            -0.0907953605055809,
            0.07467399537563324,
            -0.04727679118514061,
            -0.028494318947196007,
            -0.0278947614133358,
            0.2525108754634857,
            -0.06464426219463348,
            0.18594379723072052,
            0.13334108889102936,
            0.3466702401638031,
            0.30664315819740295,
            0.10267733037471771,
            0.04714057222008705,
            0.1208021491765976,
        ]
        similarity = round(cosine_similarity(v1, v2), 5)
        compare = round(sk_cosine_similarity([v1], [v2])[0][0], 5)
        print(similarity)
        self.assertEqual(similarity, compare)


if __name__ == "__main__":
    unittest.main()
