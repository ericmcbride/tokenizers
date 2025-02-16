from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.normalizers import Normalizer, BertNormalizer, Sequence, Lowercase, Strip


class TestBertNormalizer:
    def test_instantiate(self):
        assert isinstance(BertNormalizer(), Normalizer)
        assert isinstance(BertNormalizer(), BertNormalizer)

    def test_strip_accents(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = BertNormalizer(
            strip_accents=True, lowercase=False, handle_chinese_chars=False, clean_text=False
        )

        output = tokenizer.normalize("Héllò")
        assert output == "Hello"

    def test_handle_chinese_chars(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = BertNormalizer(
            strip_accents=False, lowercase=False, handle_chinese_chars=True, clean_text=False
        )

        output = tokenizer.normalize("你好")
        assert output == " 你  好 "

    def test_clean_text(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = BertNormalizer(
            strip_accents=False, lowercase=False, handle_chinese_chars=False, clean_text=True
        )

        output = tokenizer.normalize("\ufeffHello")
        assert output == "Hello"

    def test_lowercase(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = BertNormalizer(
            strip_accents=False, lowercase=True, handle_chinese_chars=False, clean_text=False
        )

        output = tokenizer.normalize("Héllò")
        assert output == "héllò"


class TestSequence:
    def test_instantiate(self):
        assert isinstance(Sequence([]), Normalizer)
        assert isinstance(Sequence([]), Sequence)

    def test_can_make_sequences(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = Sequence([Lowercase(), Strip()])

        output = tokenizer.normalize("  HELLO  ")
        assert output == "hello"


class TestLowercase:
    def test_instantiate(self):
        assert isinstance(Lowercase(), Normalizer)
        assert isinstance(Lowercase(), Lowercase)

    def test_lowercase(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = Lowercase()

        output = tokenizer.normalize("HELLO")
        assert output == "hello"


class TestStrip:
    def test_instantiate(self):
        assert isinstance(Strip(), Normalizer)
        assert isinstance(Strip(), Strip)

    def test_left_strip(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = Strip(left=True, right=False)

        output = tokenizer.normalize("  hello  ")
        assert output == "hello  "

    def test_right_strip(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = Strip(left=False, right=True)

        output = tokenizer.normalize("  hello  ")
        assert output == "  hello"

    def test_full_strip(self):
        tokenizer = Tokenizer(BPE())
        tokenizer.normalizer = Strip(left=True, right=True)

        output = tokenizer.normalize("  hello  ")
        assert output == "hello"
