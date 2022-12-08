using System.Diagnostics;

namespace aoc07
{
    internal abstract class ElfInode
    {
        public string Name { get; }
        public abstract long Size { get; }

        public ElfInode(string name)
        {
            Name = name;
        }

        public abstract string ToString(int indentation);

        public override string? ToString()
        {
            return ToString(0);
        }
    }

    internal class ElfFile : ElfInode
    {
        private long size;

        public ElfFile(string name, long size) : base(name)
        {
            this.size = size;
        }

        public override long Size => size;

        public override string ToString(int indentation)
        {
            return new string(' ', indentation) + $"{Name} (file, size={size})";
        }
    }

    internal class ElfDirectory : ElfInode
    {
        private List<ElfInode> contents = new();

        public ElfDirectory(string name) : base(name)
        {
        }

        public void Insert(ElfInode inode)
        {
            contents.Add(inode);
        }

        public bool Contains(string name)
        {
            return contents.Any(inode => inode.Name == name);
        }

        public override string ToString(int indentation)
        {            
            var result = new string(' ', indentation) + Name + $" (dir, size={Size})\n";
            foreach (ElfInode inode in contents)
            {
                result += inode.ToString(indentation + 2) + " \n";
            }
            return result;
        }

        public override long Size => contents.Sum(inode => inode.Size);
    }

    internal class Program
    {
        static void Main(string[] args)
        {
            const long TotalCapacity = 70000000;
            const long SpaceNeeded = 30000000;

            var input = File.ReadAllLines("real_input.txt");
            var filesystem = new Dictionary<string, ElfInode>();
            var currentDirectory = "/";
            filesystem.Add(currentDirectory, new ElfDirectory("/"));
            foreach (var line in input)
            {
                var isCommand = line.StartsWith("$");
                if (isCommand)                
                {
                    var parts = line.Split(' ');
                    if (parts[1] == "cd")
                    {
                        var argument = parts[2];
                        if (argument == "/")
                        {
                            currentDirectory = "/";
                        }
                        else if (argument == "..")
                        {
                            Debug.Assert(currentDirectory != "/");
                            var slashIndex = currentDirectory.LastIndexOf('/');
                            currentDirectory = currentDirectory.Substring(0, slashIndex);
                            if (currentDirectory.Length == 0)
                            {
                                currentDirectory = "/";
                            }
                        }
                        else
                        {
                            Debug.Assert(!argument.Contains('/'));
                            var inode = filesystem[currentDirectory];
                            Debug.Assert(inode is ElfDirectory);


                            if (!currentDirectory.EndsWith("/"))
                            {
                                currentDirectory += "/";
                            }
                            currentDirectory += argument;

                            var directory = (ElfDirectory)inode;
                            if (!directory.Contains(argument))
                            {
                                var subDirectory = new ElfDirectory(argument);
                                directory.Insert(subDirectory);
                                filesystem.Add(currentDirectory, subDirectory);
                            }
                        }
                    }
                }
                else
                {
                    // we have a file or a subdirectory
                    var parts = line.Split(' ');
                    var isDirectory = parts[0] == "dir";
                    if (isDirectory)
                    {
                        // ignore
                    }
                    else
                    {
                        // is file
                        var filesize = long.Parse(parts[0]);
                        var filename = parts[1];
                        var inode = filesystem[currentDirectory];
                        Debug.Assert(inode is ElfDirectory);
                        var directory = (ElfDirectory)inode;
                        Debug.Assert(!directory.Contains(filename));
                        directory.Insert(new ElfFile(filename, filesize));
                    }
                }
            }

            var rootDirectory = filesystem["/"];
            long freeSpace = TotalCapacity - rootDirectory.Size;
            Debug.Assert(SpaceNeeded > freeSpace);
            long toDelete = SpaceNeeded - freeSpace;
            Console.WriteLine(rootDirectory);

            /*foreach (var (directoryName, inode) in filesystem)
            {
                Console.WriteLine($"{directoryName} - {inode.Size}");
            }*/
            var part1Result = filesystem.Select(pair => pair.Value.Size).Where(size => size <= 100000).Sum();
            Console.WriteLine($"part 1 result: {part1Result}");

            var part2Result = filesystem.Select(pair => pair.Value.Size).Where(size => size >= toDelete).Min();
            Console.WriteLine($"part 2 result: {part2Result}");
        }
    }
}