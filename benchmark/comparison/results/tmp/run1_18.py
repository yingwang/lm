def flatten(lst):
    """
    Flattens a list of lists into a single list.

    Args:
        lst: A list of lists (e.g., [[1, 2], [3], [4, 5]]).

    Returns:
        A single flattened list (e.g., [1, 2, 3, 4, 5]).
    """
    return [item for sublist in lst for item in sublist]

def format_list(lst):
    """
    Formats a list into a string representation in [a, b, c] format.
    Handles boolean values as "true"/"false" and preserves decimal points for floats.

    Args:
        lst: The list to format.

    Returns:
        A string representation of the list (e.g., "[1, 2.0, true]").
    """
    formatted_items = []
    for item in lst:
        if isinstance(item, bool):
            formatted_items.append("true" if item else "false")
        elif isinstance(item, float):
            # Preserve decimal points for floats
            formatted_items.append(str(item))
        else:
            formatted_items.append(str(item))
    
    return "[" + ", ".join(formatted_items) + "]"

print(format_list(flatten([[1, 2], [3], [4, 5]])))
print(format_list(flatten([[], [1], []])))
print(format_list(flatten([])))