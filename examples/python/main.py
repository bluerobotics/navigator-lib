import navigator

tire = navigator.create_random_tire()
print(f'Tire pressure: {tire.pressure}')
print(f'Tire material: {tire.material}')
print(f'Tire size: {tire.size.width}w, {tire.size.height}h')
