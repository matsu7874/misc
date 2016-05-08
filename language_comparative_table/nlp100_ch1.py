def nlp_00():
    return 'stressed'[::-1]


def nlp_01():
    return 'パタトクカシーー'[::2]


def nlp_02():
    a = 'パトカー'
    b = 'タクシー'
    return ''.join([a[i] + b[i] for i in range(len(a))])


def nlp_03():
    s = 'Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.'
    return [len(word) for word in s.replace(',', '').replace('.', '').split()]


def nlp_04():
    s = 'Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.'
    table = {}
    for i, word in enumerate(s.replace(',', '').replace('.', '').split()):
        if i + 1 in [1, 5, 6, 7, 8, 9, 15, 16, 19]:
            table.update({word[:1]: i + 1})
        else:
            table.update({word[:2]: i + 1})
    return table


def get_ngram(sequence, n=2):
    ngrams = []
    for i in range(len(sequence) - n + 1):
        ngrams.append(sequence[i:i + n])
    return ngrams


def nlp_05():
    s = 'I am an NLPer'
    return get_ngram(s), get_ngram(s.split())


def nlp_06():
    s = ['paraparaparadise', 'paragraph']
    X = set(get_ngram(s[0]))
    Y = set(get_ngram(s[1]))

    return X | Y, X & Y, X - Y, Y - X, 'se' in X, 'se' in Y


def nlp_07():
    def nlp_07_template(x, y, z):
        return str(x) + '時の' + str(y) + 'は' + str(z)
    return nlp_07_template(12, '気温', 22.4)


def nlp_08():
    def cipher(s):
        import string
        res = ''
        for c in s:
            if c in string.ascii_lowercase:
                res += chr(219 - ord(c))
            else:
                res += c
        return res
    return cipher('日本 is very Japan.'), cipher(cipher('日本 is very Japan.'))


def nlp_09():
    s = "I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind ."

    def typoglycemia(s):
        import random
        res = []
        for word in s.split():
            if len(word) <= 4:
                res.append(word)
            else:
                mid = [c for c in word[1:-1]]
                random.shuffle(mid)
                res.append(word[0] + ''.join(mid) + word[-1])
        return ' '.join(res)
    return typoglycemia(s)


if __name__ == '__main__':
    print(nlp_00())
    print(nlp_01())
    print(nlp_02())
    print(nlp_03())
    print(nlp_04())
    print(nlp_05())
    print(nlp_06())
    print(nlp_07())
    print(nlp_08())
    print(nlp_09())
