from datetime import datetime
import json
import random

from scripts.constants import PRICE_MIN, PRICE_MAX, VOLUME_MIN, VOLUME_MAX, START, END, OVER_ORDER_STATUSES
from scripts.order_ids import gen_order_id
from timestamps import gen_random_timestamp


def gen_random_closed_order() -> dict:
	open = gen_random_timestamp(START, END)
	end = gen_random_timestamp(datetime.fromtimestamp(open), END)

	price = random.uniform(PRICE_MIN, PRICE_MAX)
	volume = random.uniform(VOLUME_MIN, VOLUME_MAX)
	executed_volume = random.uniform(VOLUME_MIN, volume)
	cost = price * executed_volume
	fee = 0.0026 * cost

	decimals = random.randint(3, 8)
	decimal_formatter = f"0.{decimals}f"

	status = random.choice(OVER_ORDER_STATUSES)

	return {
		"refid": "None",
		"userref": 1,
		"status": status,
		"reason": "User requested" if status == "canceled" else None,
		"opentm": open,
		"closetm": end,
		"starttm": 0,
		"expiretm": 0,
		"descr": {},
		"vol": format(volume, decimal_formatter),
		"vol_exec": format(executed_volume, decimal_formatter),
		"cost": format(cost, decimal_formatter),
		"fee": format(fee, decimal_formatter),
		"price": format(price, decimal_formatter),
		"stopprice": "0.00000",
		"limitprice": "0.00000",
		"misc": "",
		"oflags": "fciq",
		"trigger": "index"
	}


if __name__ == "__main__":
	orders = {gen_order_id(): gen_random_closed_order() for _ in range(3)}
	print(json.dumps(orders, indent=4))
