class Solution {
 public:
  bool checkOverlap(int radius, int x_center, int y_center, int x1, int y1,
                    int x2, int y2) {
    auto clamp = [&](int center, int mn, int mx) {
      return max(mn, min(mx, center));
    };

    int closestX = clamp(x_center, x1, x2);
    int closestY = clamp(y_center, y1, y2);

    int distanceX = x_center - closestX;
    int distanceY = y_center - closestY;

    return (distanceX * distanceX) + (distanceY * distanceY) <=
           (radius * radius);
  }
};
