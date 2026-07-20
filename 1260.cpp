class Solution {
public:
    vector<vector<int>> shiftGrid(vector<vector<int>>& grid, int k) {
        int numRows = grid.size();
        int numCols = grid[0].size();
        vector<vector<int>> result(numRows, vector<int>(numCols));
        for (int row = 0; row < numRows; ++row) {
            for (int col = 0; col < numCols; ++col) {
                int currentIndex = row * numCols + col;

                int newIndex = (currentIndex + k) % (numRows * numCols);
                int newRow = newIndex / numCols;
                int newCol = newIndex % numCols;
                result[newRow][newCol] = grid[row][col];
            }
        }
      
        return result;
    }
};
