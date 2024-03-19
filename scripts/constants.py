from datetime import datetime
import string

UPPER_ALPHA_NUMERIC = string.ascii_uppercase + string.digits
ALL_ALPHA_NUMERIC = string.ascii_uppercase + string.ascii_lowercase + string.digits
START = datetime(2020, 1, 1)
END = datetime(2024, 1, 1)

VOLUME_MIN = 0.1
VOLUME_MAX = 1000

PRICE_MIN = 0.0001
PRICE_MAX = 50_000

OPEN_ORDER_STATUSES = ["pending", "open"]
OVER_ORDER_STATUSES = ["canceled", "closed", "expired"]
ORDER_STATUSES = [*OPEN_ORDER_STATUSES, *OVER_ORDER_STATUSES]
