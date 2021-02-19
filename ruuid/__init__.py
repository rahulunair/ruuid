"""Fast UUID generator and parser for Python written in Rust.

Signature:

    def uuid() -> str

Eg.:

    from ruuid import uuid4

    uuid4()
    7a1ef475-904c-4d53-8985-528d09d57414

"""
from __future__ import absolute_import

from ruuid.ruuid import hyphenated
from ruuid.ruuid import nil
from ruuid.ruuid import simple
from ruuid.ruuid import urn
from ruuid.ruuid import uuid4

__all__ = ["hyphenated", "nil", "simple", "urn", "uuid4"]
