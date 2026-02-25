namespace Ormery.Core
{
    public class CollectedFixed<PART__7>: ICollected<PART__7>
    {
        readonly string safeText__95;
        public CollectedFixed(string safeText__97)
        {
            this.safeText__95 = safeText__97;
        }
        public string SafeText
        {
            get
            {
                return this.safeText__95;
            }
        }
    }
}
