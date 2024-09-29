import csv
import random
import string

PREFERENCES = [
    "Anime", "ANIME_", "Manga", "Mangá", "Music", "Rock", "ROCK_", "J-Pop", "Jpop", 
    "K-Pop", "Kpop", "Programming", "Technology", "Rust", "Go", "Python", "PYTHON_", 
    "Gaming", "Games", "GAMES_", "Hiking", "Cooking"
]

def generate_username():
    length = random.randint(5, 15)
    username = ''.join(random.choices(string.ascii_lowercase + string.digits, k=length))
    return username

def generate_preferences():
    return random.sample(PREFERENCES, random.randint(2, 5))

def generate_tags():
    return random.sample(PREFERENCES, random.randint(2, 5))

def generate_users_csv(file_name, num_users):
    with open(file_name, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow(["id", "username", "preferences"])

        for i in range(1, num_users + 1):
            username = generate_username()
            preferences = ", ".join(generate_preferences())
            writer.writerow([i, username, preferences])

def generate_posts_csv(file_name, num_posts):
    with open(file_name, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow(["id", "title", "tags"])

        for i in range(1, num_posts + 1):
            title = f"Post: #{i}: {generate_username()}'s Content"
            tags = ", ".join(generate_tags())
            writer.writerow([i, title, tags])

generate_users_csv("src/csv/users.csv", 10000)
generate_posts_csv("src/csv/posts.csv", 10000)

print("Generated fake database")