from datetime import datetime
from kybra import query


@query
def get_date(isoString: str) -> int:
    return datetime.fromisoformat(isoString).day


@query
def get_day(isoString: str) -> int:
    return datetime.fromisoformat(isoString).weekday()


@query
def get_full_year(isoString: str) -> int:
    return datetime.fromisoformat(isoString).year


@query
def get_hours(isoString: str) -> int:
    return datetime.fromisoformat(isoString).hour


@query
def get_milliseconds(isoString: str) -> int:
    dt = datetime.fromisoformat(isoString)
    return dt.microsecond // 1000


@query
def get_minutes(isoString: str) -> int:
    return datetime.fromisoformat(isoString).minute


@query
def get_month(isoString: str) -> int:
    return datetime.fromisoformat(isoString).month


@query
def get_seconds(isoString: str) -> int:
    return datetime.fromisoformat(isoString).second


@query
def get_time(isoString: str) -> int:
    dt = datetime.fromisoformat(isoString)
    return int(dt.timestamp() * 1000)


@query
def get_timezone_offset(isoString: str) -> int:
    dt = datetime.fromisoformat(isoString)
    utcoffset = dt.utcoffset()

    if utcoffset is None:
        return 0
    else:
        return int(utcoffset.total_seconds() / 60) if dt.tzinfo else 0
