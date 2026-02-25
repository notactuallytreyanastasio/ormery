namespace Ormery
{
    public class WhereClause
    {
        readonly string field__360;
        readonly string operator__361;
        readonly string value__362;
        public string Describe()
        {
            return this.field__360 + " " + this.operator__361 + " " + this.value__362;
        }
        public WhereClause(string field__366, string operator__367, string value__368)
        {
            this.field__360 = field__366;
            this.operator__361 = operator__367;
            this.value__362 = value__368;
        }
        public string Field
        {
            get
            {
                return this.field__360;
            }
        }
        public string Operator
        {
            get
            {
                return this.operator__361;
            }
        }
        public string Value
        {
            get
            {
                return this.value__362;
            }
        }
    }
}
