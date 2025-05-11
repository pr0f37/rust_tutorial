# Analysis

We can think of this problem in two main possible ways:

1. we keep the list organised at all times
2. we sort the list only when we need to find the median
   Both strategies will work and both have their advantages and disadvantages.

## List organised all the time

We start with empty list - this means we may keep it sorted if we'll be adding new items in right places, maintaining the order.

Since the list is always sorted we just want to find a right place to add new item.
The most performant algorithm to find and item in the sorted list is binary
search. This takes O(logn).
Finding median in sorted list is stable O(1).

Overall complexity for n inserts and m median reads is O(nlogn + m) => O(nlogn).

## Sorting the list when searching for the median

We add new elements always at the end of the list. When we want median we sort the list and return the median element.
The most performant sorting algorithm is quicksort and it takes O(nlogn).

Overall complexity for n inserts and m median reads is O(n + mnlogn) => O(mnlogn)
