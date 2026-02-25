using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace Ormery.Core
{
    public class Collector<PART__2>
    {
        readonly G::IList<ICollected<PART__2>> partsBuilder__85;
        public void AppendSafe(string fixed__87)
        {
            CollectedFixed<PART__2> t___519 = new CollectedFixed<PART__2>(fixed__87);
            C::Listed.Add(this.partsBuilder__85, t___519);
        }
        public void Append(PART__2 part__90)
        {
            CollectedPart<PART__2> t___517 = new CollectedPart<PART__2>(part__90);
            C::Listed.Add(this.partsBuilder__85, t___517);
        }
        public G::IReadOnlyList<ICollected<PART__2>> Parts
        {
            get
            {
                return C::Listed.ToReadOnlyList(this.partsBuilder__85);
            }
        }
        public Collector()
        {
            G::IList<ICollected<PART__2>> t___515 = new G::List<ICollected<PART__2>>();
            this.partsBuilder__85 = t___515;
        }
    }
}
