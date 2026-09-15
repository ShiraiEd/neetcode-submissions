class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        s = sum([b * b for b in s.encode()])
        t = sum([b * b for b in t.encode()])
        if s == t:
            return True
        return False

        