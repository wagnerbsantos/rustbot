import pyautogui
import numpy as np
import os
import threading
import time
import keyboard
import random
import traceback
import cv2
from PIL import Image
from consts import *

ranges = []
# food = 0
random.seed()
pyautogui.PAUSE = 0.01
dst = None
#winname = 'window'
#cv2.namedWindow(winname)

class Status:
    is_attacking = False
    life_value = 3
    mana_value = 3
    has_enemy = False
    is_following = False
    food_time = -2
    stopped_attacking = False
    looted = False
    isMoving = False
    attack_cooldown = 0
    item_cooldown = 0
    heal_cooldown = 0
    move_cooldown = 0




def cropper(image):
    return image[ MAP_TOP_LEFT[1]: MAP_HEIGHT + MAP_TOP_LEFT[1] , MAP_TOP_LEFT[0]: MAP_WIDTH + MAP_TOP_LEFT[0],]


def averager(cropped):
    r = 0
    g = 0
    b = 0
    q = 1
    for line in cropped:
        for pixel in line:
            r = r + pixel[0]
            g = g + pixel[1]
            b = b + pixel[2]
            q = q + 1
    return singleRange((r // q, g // q, b // q), cropped)


def ranger(averages):
    count = 0
    for line in averages:
        for average in line:
            inrange = False
            for r in ranges:
                if isInRange(average, r):
                    inrange = True
                    break
            if not inrange:
                ranges.append(average)
    return ranges


def singleRange(rgb, image):
    inrange = False
    ran = (0, 0, 0)
    for r in ranges:
        if isInRange(rgb, r):
            inrange = True
            ran = r
            break
    if not inrange:
        ranges.append(rgb)
        ran = rgb
    return ran


def isInRange(average, ran):
    offset = 10
    for i in range(3):
        if not (average[i] > ran[i] - offset and average[i] < ran[i] + offset):
            return False

    return True


def getIsAttacking(image):
    if image[IS_ATTACKING_NEAREST_POS1][RED] > IS_ATTACKING_RED:
        return True
    if image[IS_ATTACKING_NEAREST_POS2][RED] > IS_ATTACKING_RED:
        return True
    if image[IS_ATTACKING_NEAREST_POS3][RED] > IS_ATTACKING_RED:
        return True
    if image[IS_ATTACKING_NEAREST_POS4][RED] > IS_ATTACKING_RED:
        return True
    return False


def getLifeValue(image):
    if image[HIGH_LIFE_POS][GREEN] == HAS_LIFE_GREEN:
        return 3
    elif image[MID_LIFE_POS][GREEN] == HAS_LIFE_GREEN:
        return 2
    elif image[LOW_LIFE_POS][GREEN] == HAS_LIFE_GREEN:
        return 1
    return 0


def getManaValue(image):
    if image[HIGH_MANA_POS][BLUE] == HAS_MANA_BLUE:
        return 3
    elif image[MID_MANA_POS][BLUE] == HAS_MANA_BLUE:
        return 2
    elif image[LOW_MANA_POS][BLUE] == HAS_MANA_BLUE:
        return 1
    return 0


def getHasEnemy(image):
    if tuple(image[FIRST_ENEMY_POS][0:3]) == ENEMY_EXIST_RGB:
        return True
    return False


def getIsFollowing(image):
    if image[FOLLOW_BUTTON][GREEN] > 250:
        return True
    return False


def getStatus(status: Status, image):
    is_attacking = getIsAttacking(image)
    if status.is_attacking and not is_attacking:
        status.stopped_attacking = True
    status.is_attacking = is_attacking
    status.life_value = getLifeValue(image)
    status.mana_value = getManaValue(image)
    status.has_enemy = getHasEnemy(image)
    status.is_following = getIsFollowing(image)
    status.food_time += 1
    status.attack_cooldown -= 0.2
    status.heal_cooldown -= 0.2
    status.item_cooldown -= 0.2
    status.move_cooldown -= 0.2


def loot():
    pyautogui.keyDown("shift")
    for square in LOOT_SQUARES:
        x = square[0] 
        y = square[1]
        pyautogui.rightClick(x, y)
    for square in reversed(LOOT_SQUARES):
        x = square[0] 
        y = square[1]
        pyautogui.rightClick(x, y)
    pyautogui.keyUp("shift")
    pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)))


def decision(status: Status, walkables: list):
    if status.stopped_attacking:
        status.looted = True
        loot()
        status.stopped_attacking = False
    if status.life_value < 3  and status.heal_cooldown < 0:
       pyautogui.press(HEAL_HOTKEY)
       status.heal_cooldown = 1.2
    elif status.mana_value == 3 and status.attack_cooldown < 0:
        pyautogui.press(MANA_WASTE)
        status.attack_cooldown = 3
    elif ALLOW_WASTE and status.attack_cooldown < 0:
        pyautogui.press(MANA_WASTE)
        status.attack_cooldown = 3
    if (status.mana_value == 0) and status.item_cooldown < 0:
        pyautogui.press(MANA_POTION_WASTE)
        status.item_cooldown = 3
    if status.food_time % 80*5*30 == 0:
        time.sleep(1)
        pyautogui.press(FOOD_HOTKEY)
        time.sleep(1)
        pyautogui.press(EXTRA_FOOD)
        status.food_time = 1 
    if status.has_enemy and not status.is_attacking:
        pyautogui.click(tuple(reversed(FIRST_ENEMY_POS)))
        pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)))
    if (
        not status.isMoving
        and not status.is_attacking
        and not status.has_enemy
        and status.move_cooldown< 0
    ):
        move(walkables)
        status.move_cooldown = 2
    if status.is_attacking and not status.is_following:
        pyautogui.click(tuple(reversed(FOLLOW_BUTTON)))
        pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)))


def move(walkables):
    print(walkables)
    if len(walkables) < 2:
        return
    numero = random.randint(0, len(walkables) - 1)
    pyautogui.click(tuple(reversed(walkables[numero])))
    pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)), duration=0.1)


def findWalkable(image):
    listao = []
    basew = MAP_TOP_LEFT[0]
    baseh = MAP_TOP_LEFT[1]
    se = set()
    for h in range(5, MAP_HEIGHT, 5):
        for w in range(5, MAP_WIDTH, 5):
            if h < 45 and h > 25 and w < 45 and w > 25:
                continue
            se.add(tuple(image[baseh + h][basew + w][0:3]))
            if tuple(image[baseh + h][basew + w][0:3]) in MAP_WALKABLE_RGB:
                listao.append((baseh + h, basew + w))
    #print(se)
    return listao


def clearFiles():
    files = os.listdir()
    for file in files:
        os.remove(file)

def isMoving(minimap, dst):
    mask = cv2.threshold(minimap, 150, 255, cv2.THRESH_BINARY)[1][:, :, 0]
    if dst is None:
        dst = mask.copy().astype(float)
    mask = mask.copy().astype(float)
    weighted = cv2.accumulateWeighted(mask, dst, 0.5)
    btnot = cv2.absdiff(weighted, mask)
    white = np.sum(btnot[btnot>0.01])
    return dst, white > 10


def main():
    status = Status()
    dst = None
    while True:
        im = np.array(pyautogui.screenshot())
        minimap = cropper(im)
        dst, status.isMoving = isMoving(minimap, dst)
        getStatus(status, im)
        print(status.isMoving, status.is_attacking, status.has_enemy, status.move_cooldown)
        walkables = findWalkable(im)
        decision(status, walkables)
        time.sleep(0.2)
        if status.looted:
            status.looted = False


if __name__ == "__main__":
    main()                          
