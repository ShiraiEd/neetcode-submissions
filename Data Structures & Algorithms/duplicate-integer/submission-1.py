class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        set_l = set()
        for n in nums:
            if n in set_l:
                return True
            set_l.add(n)
        return False
        