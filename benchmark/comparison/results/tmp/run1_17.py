class Pair:
    """
    A simple class to represent a pair of integers.
    Provides a custom __repr__ for formatted output.
    """
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        """
        Returns the string representation of the Pair object, e.g., "Pair(1, 4)".
        """
        return f"Pair({self.x}, {self.y})"

def zip_lists(a: list[int], b: list[int]) -> list[Pair]:
    """
    Pairs elements from two integer lists, stopping at the length of the shorter list.
    
    Args:
        a: The first list of integers.
        b: The second list of integers.
        
    Returns:
        A list of Pair objects, where each Pair contains elements from corresponding
        positions in the input lists.
    """
    result = []
    min_len = min(len(a), len(b))
    for i in range(min_len):
        result.append(Pair(a[i], b[i]))
    return result

def format_pair_list(pair_list: list[Pair]) -> str:
    """
    Formats a list of Pair objects into a string representation like
    '[Pair(1, 4), Pair(2, 5)]'.
    
    Args:
        pair_list: A list of Pair objects.
        
    Returns:
        A string representing the list of pairs.
    """
    pair_strings = [repr(p) for p in pair_list]
    return f"[{', '.join(pair_strings)}]"

print(format_pair_list(zip_lists([1, 2, 3], [4, 5, 6])))
print(format_pair_list(zip_lists([1, 2], [3])))
print(format_pair_list(zip_lists([], [1])))