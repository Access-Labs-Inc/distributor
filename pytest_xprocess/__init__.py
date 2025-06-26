# Compatibility shim for anchorpy
# This provides the import path that anchorpy expects

def getrootdir(config):
    """
    Dummy implementation of getrootdir for anchorpy compatibility.
    Since we removed pytest-xprocess, this provides a minimal implementation.
    """
    import tempfile
    import os
    return os.path.join(tempfile.gettempdir(), '.pytest_cache') 