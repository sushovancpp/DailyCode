class Solution {
    public:
    int maximumElementAfterDecrementingAndRearranging(vector<int>& arr) {
        int n=arr.size();
        sort(arr.begin(),arr.end());
        int maxi(-1);
        arr[0]=1;
        for(int i=0;i<n-1;i++)
        {
            if(arr[i]==arr[i+1])
            {
                maxi=max(maxi,arr[i]);
                continue;
            }
            else
            {
                arr[i+1]=arr[i]+1;
                maxi=max(maxi,arr[i]);
            }
        }
        maxi=max(maxi,arr[n-1]);
        return maxi;
    }
};
