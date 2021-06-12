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
            0.08306299149990082,
            0.018646100535988808,
            -0.10669267922639847,
            0.12530650198459625,
            -0.05736806243658066,
            0.20346316695213318,
            -0.014108438044786453,
            0.13136503100395203,
            0.10572922229766846,
            -0.03349698334932327,
            -0.02693246118724346,
            0.04800524190068245,
            -0.12470601499080658,
            0.09357196092605591,
            -0.005935535300523043,
            0.1502171903848648,
            0.01678784377872944,
            0.008319453336298466,
            -0.04493426904082298,
            -0.11556509882211685,
        ]
        similarity = round(cosine_similarity(v1, v2), 5)
        compare = round(sk_cosine_similarity([v1], [v2])[0][0], 5)
        self.assertEqual(similarity, compare)


if __name__ == "__main__":
    unittest.main()
