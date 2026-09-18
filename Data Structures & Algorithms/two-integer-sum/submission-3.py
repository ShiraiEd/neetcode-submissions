class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hash = {}
        result = []
        for i, v in enumerate(nums):
            gap = target -  v
            if gap in hash:
                result.append(hash[gap])
                result.append(i)
            else :
                hash[v] = i
        return result