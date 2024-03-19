from random import choices

from scripts.constants import ALL_ALPHA_NUMERIC


def gen_random_kraken_transfer_identifiers(n=10):
	return [
		f"{gen_segment(7)}-{gen_segment(22)}"
		for _ in range(n)
	]


def gen_segment(n: int) -> str:
	return "".join(choices(ALL_ALPHA_NUMERIC, k=n))


if __name__ == "__main__":
	for uuid in gen_random_kraken_transfer_identifiers(3):
		print(uuid)
