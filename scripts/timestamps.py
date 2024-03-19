from datetime import datetime
from random import random


def gen_random_timestamp(start: datetime, end: datetime) -> float:
	diff = end.timestamp() - start.timestamp()
	rand = random() * diff
	return start.timestamp() + rand


if __name__ == "__main__":
	for _ in range(11):
		ts = gen_random_timestamp(datetime(2020, 1, 1), datetime(2024, 1, 1))
		print(ts)
