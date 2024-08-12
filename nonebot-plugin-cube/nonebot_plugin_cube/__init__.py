import time
from typing import Dict

from nonebot import on_command
from nonebot.adapters.onebot.v11 import MessageSegment
from nonebot.adapters import Message
from nonebot.params import CommandArg
from .render import DrawCube
from cube_rs import CubeCore
from nonebot.adapters.onebot.v11 import GroupMessageEvent
from .rank import add_point, get_rank, get_point

game = on_command('魔方', aliases={"mf", "cube", "c"}, priority=20)
group_id_list = []  # 记录当前已开游戏的群号的列表
obj_dist: Dict[int, CubeCore] = {}  # 记录魔方对象

@game.handle()
async def _(event: GroupMessageEvent, args: Message = CommandArg()):
    group_id = event.group_id
    uid = event.user_id

    if group_id not in group_id_list:
        group_id_list.append(group_id)
        cube = CubeCore()  # 实例化魔方
        cube.scramble(1000)  # 打乱魔方
        buf = DrawCube(cube.get_cube()).get_buf()
        msg = MessageSegment.image(buf.getvalue())
        obj_dist[group_id] = cube
        await game.finish(msg)
        buf.close()

    cube = obj_dist[group_id]

    if args.extract_plain_text() == 'bk':  # 撤销操作
        if len(cube.get_last_step()) == 0:
            await game.finish('已撤销为最初状态！')
        plain_texts: str = cube.get_last_step()
        for plain_text in plain_texts[::-1]:
            meth = ''
            if plain_text.islower():
                meth += plain_text
                cube.rotate(plain_text.upper())
            else:
                meth += plain_text
                cube.rotate(plain_text.lower())
        buf = DrawCube(cube.get_cube()).get_buf()
        msg = MessageSegment.image(buf.getvalue())
        await game.finish(f"撤销操作:{''.join(plain_texts)}" + msg)
        buf.close()
    if args.extract_plain_text() == '结束':
        group_id_list.remove(group_id)
        await game.finish("游戏结束")
    plain_texts = args.extract_plain_text()  # 命令匹配
    cube.rotate(plain_texts)
    command_list = []
    if cube.is_solved():
        buf = DrawCube(cube.get_cube()).get_buf()
        msg = MessageSegment.image(buf.getvalue())
        group_id_list.remove(group_id)
        dt = duration(cube.get_start_time())
        add_point(uid=uid, group_id=group_id, name=event.sender.nickname)  # 添加积分
        point = get_point(uid=uid, group=group_id)  # 获取当前积分
        await game.finish(f"已还原，用时{dt}\n获得积分1,当前积分{point}" + msg)  # 发送当前积分
    buf = DrawCube(cube.get_cube()).get_buf()
    msg = MessageSegment.image(buf.getvalue())
    dt = duration(cube.get_start_time())
    await game.finish(f"已执行操作:{cube.get_last_step()},时间{dt}" + msg)
    buf.close()


rank = on_command("rank", aliases={'排名', '排行榜'}, priority=20)


@rank.handle()
async def send_rank(event: GroupMessageEvent):  # 发送群排名
    rank_text = get_rank(event.group_id)
    await rank.finish(rank_text)


def duration(start_time: int) -> str:
    dt = (time.time()*1000 - start_time)/1000
    ms = str(dt).split('.')[1][:3]
    return time.strftime('%H:%M:%S:', time.gmtime(dt)) + ms
