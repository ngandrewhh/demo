'''
There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length),
such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). 
For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

Given the array nums after the rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.
''' 

def pvtSearch(ll, idx = 0):
    #print(ll, ll[len(ll)//2], idx)
    
    if (len(ll) < 2):
        #print(ll, idx)
        return ll, idx
    
    if ll[len(ll)//2] > ll[0]:
        return pvtSearch(ll[len(ll)//2:], idx  + len(ll)//2)
    else:
        return pvtSearch(ll[:len(ll)//2], idx)
    
def binSearch(ll, target, idx = 0):

    #print(ll)
    if not ll: return -1
    if ll[len(ll)//2] == target: return idx + len(ll)//2
    if len(ll) < 2: return -1
    if target > ll[len(ll)//2]:
        return binSearch(ll[len(ll)//2 + 1:], target, idx + len(ll)//2 + 1)
    else:
        return binSearch(ll[:len(ll)//2], target, idx)
    
def search(nums, target):
    if len(nums) == 1:
        if target == nums[0]: return 0

    _, pvt = pvtSearch(nums)
    #print("pvt: {pvt}".format(pvt = pvt))

    res    = binSearch(nums[pvt+1:] + nums[:pvt+1], target)
    #print("res: {res}".format(res = res))
    if res == -1:
        return res
    else:
        if pvt == (len(nums) - 1): pvt = -1
        return (res + pvt + 1) % len(nums)

    
    
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        if len(nums) == 1:
            if target == nums[0]: return 0
            
        _, pvt = pvtSearch(nums)
        #print("pvt: {pvt}".format(pvt = pvt))
        
        res    = binSearch(nums[pvt+1:] + nums[:pvt+1], target)
        #print("res: {res}".format(res = res))
        if res == -1:
            return res
        else:
            if pvt == (len(nums) - 1): pvt = -1
            return (res + pvt + 1) % len(nums)
        
        
