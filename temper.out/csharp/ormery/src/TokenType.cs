namespace Ormery
{
    public class TokenType
    {
        readonly string name__657;
        public bool IsKeyword
        {
            get
            {
                return this.name__657 == "keyword";
            }
        }
        public bool IsType
        {
            get
            {
                return this.name__657 == "type";
            }
        }
        public bool IsString
        {
            get
            {
                return this.name__657 == "string";
            }
        }
        public bool IsNumber
        {
            get
            {
                return this.name__657 == "number";
            }
        }
        public bool IsComment
        {
            get
            {
                return this.name__657 == "comment";
            }
        }
        public bool IsOperator
        {
            get
            {
                return this.name__657 == "operator";
            }
        }
        public bool IsIdentifier
        {
            get
            {
                return this.name__657 == "identifier";
            }
        }
        public TokenType(string name__673)
        {
            this.name__657 = name__673;
        }
        public string Name
        {
            get
            {
                return this.name__657;
            }
        }
    }
}
