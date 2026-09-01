class Solution {
public:
    int minMoves(vector<string>& classroom, int energy) {
        int rows = classroom.size();
        int cols = classroom[0].size();
        vector<vector<int>> lightIndex(rows, vector<int>(cols, 0));
        int startRow = 0, startCol = 0;
        int lightCount = 0;

        for (int i = 0; i < rows; ++i) {
            string& currentRow = classroom[i];
            for (int j = 0; j < cols; ++j) {
                char cell = currentRow[j];
                if (cell == 'S') {

                    startRow = i;
                    startCol = j;
                } else if (cell == 'L') {
                    lightIndex[i][j] = lightCount;
                    lightCount++;
                }
            }
        }

        if (lightCount == 0) {
            return 0;
        }

        vector<vector<vector<vector<bool>>>> visited(rows, 
            vector<vector<vector<bool>>>(cols, 
                vector<vector<bool>>(energy + 1, 
                    vector<bool>(1 << lightCount, false))));

        queue<tuple<int, int, int, int>> bfsQueue;

        int allLightsOn = (1 << lightCount) - 1;
        bfsQueue.emplace(startRow, startCol, energy, allLightsOn);
        visited[startRow][startCol][energy][allLightsOn] = true;
      

        vector<int> directions = {-1, 0, 1, 0, -1};
      
        int moves = 0;

        while (!bfsQueue.empty()) {
            int levelSize = bfsQueue.size();

            while (levelSize--) {
                auto [currentRow, currentCol, currentEnergy, lightMask] = bfsQueue.front();
                bfsQueue.pop();

                if (lightMask == 0) {
                    return moves;
                }

                if (currentEnergy <= 0) {
                    continue;
                }

                for (int dir = 0; dir < 4; ++dir) {
                    int nextRow = currentRow + directions[dir];
                    int nextCol = currentCol + directions[dir + 1];

                    if (nextRow >= 0 && nextRow < rows && 
                        nextCol >= 0 && nextCol < cols && 
                        classroom[nextRow][nextCol] != 'X') {
                      

                        int nextEnergy = (classroom[nextRow][nextCol] == 'R') ? 
                                        energy : currentEnergy - 1;
                      

                        int nextLightMask = lightMask;
                        if (classroom[nextRow][nextCol] == 'L') {
                            nextLightMask &= ~(1 << lightIndex[nextRow][nextCol]);
                        }

                        if (!visited[nextRow][nextCol][nextEnergy][nextLightMask]) {
                            visited[nextRow][nextCol][nextEnergy][nextLightMask] = true;
                            bfsQueue.emplace(nextRow, nextCol, nextEnergy, nextLightMask);
                        }
                    }
                }
            }
            moves++;
        }

        return -1;
    }
};
