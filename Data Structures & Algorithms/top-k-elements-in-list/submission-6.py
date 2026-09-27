class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        if len(nums) == 1 and k == 1:
            return nums
        freq = {}
        for n in nums:
            if n not in freq:
                freq[n] = 0
            elif n in freq:
                freq[n] += 1
        buckets: list[list[int]] = [[] for _ in range((len(nums) + 1))]
        for n, f in freq.items():
            buckets[f].append(n)
        res: list[int] = []

        for b in range(len(nums), -1, -1):
            for i in buckets[b]:
                res.append(i)
                if len(res) == k:
                    return res
            
        return res