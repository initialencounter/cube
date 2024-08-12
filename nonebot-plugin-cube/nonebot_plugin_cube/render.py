from PIL import Image, ImageDraw
import io
from cube_rs import CubeCore


class DrawCube:
    def __init__(self, newmf):
        self.lst = [newmf[4],newmf[2],newmf[0],newmf[3],newmf[1],newmf[5]]
        self.face_id = [[0, 1], [1, 0], [1, 1], [1, 2], [1, 3], [2, 1]]
        self.color = {
            1: 'red', 2: 'red', 3: 'red', 4: 'red', 5: 'red', 6: 'red', 7: 'red', 8: 'red', 9: 'red',
            10: 'blue', 11: 'blue', 12: 'blue', 13: 'blue', 14: 'blue', 15: 'blue', 16: 'blue', 17: 'blue', 18: 'blue',
            19: 'yellow', 20: 'yellow', 21: 'yellow', 22: 'yellow', 23: 'yellow', 24: 'yellow', 25: 'yellow',
            26: 'yellow', 27: 'yellow',
            28: 'orange', 29: 'orange', 30: 'orange', 31: 'orange', 32: 'orange', 33: 'orange', 34: 'orange',
            35: 'orange', 36: 'orange',
            37: 'green', 38: 'green', 39: 'green', 40: 'green', 41: 'green', 42: 'green', 43: 'green', 44: 'green',
            45: 'green',
            46: 'white', 47: 'white', 48: 'white', 49: 'white', 50: 'white', 51: 'white', 52: 'white', 53: 'white',
            54: 'white'
        }
        self.img = Image.new('RGB', (525, 275), color='black')

    def draw(self, dx, dy, arr):
        drawer = ImageDraw.Draw(self.img)
        dx = dx * 100
        dy = dy * 100
        cons, conty = 25, 0
        for i in arr:
            contx = 0
            for j in i:
                posx = contx * cons + dx
                posy = conty * cons + dy
                drawer.rectangle((posx, posy, posx + 20, posy + 20), fill=self.color[j])
                contx += 1
            conty += 1

    def draw_all_cube(self):
        for i in range(len(self.lst)):
            self.draw(self.face_id[i][1], self.face_id[i][0], self.lst[i])

    def prjctn(self):
        #     画投影图
        draw = ImageDraw.Draw(self.img)
        dx, dy, cons, conty = 400, 100, 25, 0
        for i in self.lst[2]:
            contx = 0
            for j in i:
                posx = contx * cons + dx
                posy = conty * cons + dy
                draw.rectangle((posx, posy, posx + 20, posy + 20), fill=self.color[j])
                contx += 1
            conty += 1
        dx = 470
        dy = 84
        for i in self.lst[0][::-1]:
            for j in i[::-1]:
                draw.polygon([(11 + dx, 0 + dy), (-9 + dx, 0 + dy), (-20 + dx, 11 + dy), (0 + dx, 11 + dy)],
                             fill=self.color[j])
                dx -= 25
            dy -= 14
            dx += 91
        dx = 474
        dy = 87
        for i in self.lst[3]:
            for j in i:
                draw.polygon([(11 + dx, 0 + dy), (0 + dx, 11 + dy), (0 + dx, 31 + dy), (11 + dx, 20 + dy)],
                             fill=self.color[j])
                dx += 16
                dy -= 13
            dx = 474
            dy += 64.5

    def get_buf(self):
        self.draw_all_cube()
        self.prjctn()
        image = self.img
        buf = io.BytesIO()
        image.save(buf, format='png')
        return buf


if __name__ == '__main__':
    cube = CubeCore()
    cube.rotate("F")
    dr = Draw_cube(cube.get_cube())
    dr.draw_all_cube()
    dr.prjctn()
    img = dr.img
    img.save('./test.png')
