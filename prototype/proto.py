import json, os, sys



def main():
    sys.stdout.write("loading config\n")
    with open("config.json") as file:
        config = json.load(file)

    sys.stdout.write("creating output\n")
    output = open(config["output file"],"w")

    for file in os.listdir(config["input folder"]):
        sys.stdout.write(f"opening {file}... ") 
        with open(config["input folder"] +"/"+file) as inputFile:
            for line in inputFile:
                if len(line) >= config["filter"]["min"] :
                    output.write(line) 
        
        output.write("\n") 
        sys.stdout.write(f"done\n") 
    
    sys.stdout.write(f"closing output\n") 
    output.close()


if __name__ == "__main__":
    main()