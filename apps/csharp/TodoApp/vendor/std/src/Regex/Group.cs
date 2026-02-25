namespace TemperLang.Std.Regex
{
    public class Group
    {
        readonly string name__242;
        readonly string value__243;
        readonly int begin__244;
        readonly int end__245;
        public Group(string name__247, string value__248, int begin__249, int end__250)
        {
            this.name__242 = name__247;
            this.value__243 = value__248;
            this.begin__244 = begin__249;
            this.end__245 = end__250;
        }
        public string Name
        {
            get
            {
                return this.name__242;
            }
        }
        public string Value
        {
            get
            {
                return this.value__243;
            }
        }
        public int Begin
        {
            get
            {
                return this.begin__244;
            }
        }
        public int End
        {
            get
            {
                return this.end__245;
            }
        }
    }
}
