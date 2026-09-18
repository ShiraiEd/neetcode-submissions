class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hash = {}
        for i, v in enumerate(nums):
            gap = target -  v
            if gap in hash:
                return [hash[gap], i]
            hash[v] = i
        return []