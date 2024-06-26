from typing import Tuple

import numpy as np
import numpy.typing as npt

__version__: str = ...

def py_counting(
    m: npt.NDArray[np.int64]
) -> npt.NDArray[np.int64]: ...

def py_bradley_terry(
    m: npt.NDArray[np.int64]
) -> Tuple[npt.NDArray[np.float64], int]: ...

def py_newman(
    m: npt.NDArray[np.int64],
    seed: int,
    tolerance: float,
    max_iter: int
) -> Tuple[npt.NDArray[np.float64], int]: ...
