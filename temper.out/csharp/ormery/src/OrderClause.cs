namespace Ormery
{
    public class OrderClause
    {
        readonly string field__369;
        readonly string direction__370;
        public string Describe()
        {
            return this.field__369 + " " + this.direction__370;
        }
        public OrderClause(string field__374, string direction__375)
        {
            this.field__369 = field__374;
            this.direction__370 = direction__375;
        }
        public string Field
        {
            get
            {
                return this.field__369;
            }
        }
        public string Direction
        {
            get
            {
                return this.direction__370;
            }
        }
    }
}
