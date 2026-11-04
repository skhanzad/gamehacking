from gamehack import scan_bytes


def test_scan_bytes():
    assert scan_bytes(list(b"abcabc"), list(b"bc")) == [1, 4]
