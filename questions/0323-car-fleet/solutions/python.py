def car_fleet(target: int, position: list[int], speed: list[int]) -> int:
    cars = sorted(zip(position, speed), reverse=True)
    fleets = 0
    lead_dist, lead_speed = 0, 1
    for p, s in cars:
        dist = target - p
        if dist * lead_speed > lead_dist * s:
            fleets += 1
            lead_dist, lead_speed = dist, s
    return fleets
