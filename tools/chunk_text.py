import spacy, sys

nlp = spacy.load("en_core_web_sm")
text = open("data/paradise_lost.txt").read()

FUNCTION = {"DET", "ADP", "CCONJ", "SCONJ", "PRON", "AUX", "PART"}

with open("data/chunks.txt", "w") as out:
    for doc in nlp.pipe(text.split("\n\n")):
        words = [t for t in doc if not t.is_punct and not t.is_space]
        i = 0
        while i < len(words):
            for n in (3, 2, 1):
                if i + n > len(words):
                    continue
                group = words[i:i+n]
                if group[-1].pos_ in FUNCTION:
                    continue
                if group[0].pos_ == "PART":
                    continue
                if not any(t.pos_ not in FUNCTION for t in group):
                    continue
                out.write(" ".join(t.text for t in group) + "\n")
                i += n
                break
            else:
                i += 1