# Panel QA Tests

## Parking Panel - WORKS
- Shows "Smart Parking" header with Hebrew subtitle
- 502 available nearby, 6 locations
- Lists: בזל (55%), Parking 2 (56%), Parking 3 (58%), Parking 4 (19%), גן העיר (21%), תל נורדאו (59%)
- Each has distance, price, capacity, type (Surface/Underground/Multi-Storey)
- Progress bars with color coding (red=low, orange=medium, green=high)
- Sparkline charts for each
- FPS dropped to 16fps when panel open

## Issues
- FPS drops when panels open (16fps vs 37fps on main screen)
- Panel names "Parking 2", "Parking 3", "Parking 4" are generic - should be real names
