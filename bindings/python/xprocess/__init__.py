# Compatibility shim for anchorpy
# This provides the classes that anchorpy expects from xprocess

class ProcessStarter:
    """Dummy ProcessStarter class for anchorpy compatibility."""
    pass

class XProcessInfo:
    """Dummy XProcessInfo class for anchorpy compatibility."""
    def terminate(self, timeout=60):
        """Dummy terminate method."""
        return 0

class XProcess:
    """Dummy XProcess class for anchorpy compatibility."""
    def __init__(self, config, rootdir):
        self.config = config
        self.rootdir = rootdir
    
    def __enter__(self):
        return self
    
    def __exit__(self, *args):
        pass
    
    def ensure(self, name, starter):
        """Dummy ensure method."""
        return (12345, "/tmp/logfile")
    
    def getinfo(self, name):
        """Dummy getinfo method."""
        return XProcessInfo()
