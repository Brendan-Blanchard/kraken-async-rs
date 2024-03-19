
from random import choices

from constants import UPPER_ALPHA_NUMERIC


def gen_random_kraken_identifiers(n=10):
	return [gen_order_id() for _ in range(n)]


def gen_order_id() -> str:
	return f"{gen_segment(6)}-{gen_segment(5)}-{gen_segment(6)}"


def gen_segment(n: int) -> str:
	return "".join(choices(UPPER_ALPHA_NUMERIC, k=n))


if __name__ == "__main__":
	for uuid in gen_random_kraken_identifiers(19):
		print(uuid)
