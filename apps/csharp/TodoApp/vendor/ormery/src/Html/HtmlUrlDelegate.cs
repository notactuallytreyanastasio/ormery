using C = Ormery.Core;
using H = Ormery.Html;
using S = System;
namespace Ormery.Html
{
    public class HtmlUrlDelegate: C::IContextDelegate<UrlEscaperContext>, IHtmlDelegate
    {
        C::AutoescState<UrlEscaperContext> _state__677;
        C::Subsidiary ? _subsidiary__678;
        public C::AutoescState<UrlEscaperContext> State
        {
            get
            {
                return this._state__677;
            }
            set
            {
                this._state__677 = value;
            }
        }
        public C::IContextPropagator<UrlEscaperContext> ContextPropagator
        {
            get
            {
                return H::HtmlGlobal.urlContextPropagator__268;
            }
        }
        public IHtmlEscaper Escaper(IHtmlEscaper outer__687)
        {
            IHtmlEscaper return__229;
            bool t___3102;
            int t___5402 = this.State.Context.UrlState;
            if (t___5402 == 0)
            {
                return__229 = new HtmlUrlEscaperAdapter(H::HtmlGlobal.htmlProtocolFilteringUrlEscaper__295, outer__687);
            }
            else if (t___5402 == 1)
            {
                return__229 = new HtmlUrlEscaperAdapter(H::HtmlGlobal.htmlUrlPartUrlEscaper__298, outer__687);
            }
            else
            {
                if (t___5402 == 2)
                {
                    t___3102 = true;
                }
                else
                {
                    t___3102 = t___5402 == 3;
                }
                if (t___3102)
                {
                    return__229 = new HtmlUrlEscaperAdapter(H::HtmlGlobal.htmlAsIfQueryUrlEscaper__299, outer__687);
                }
                else throw new S::Exception();
            }
            return return__229;
        }
        public HtmlUrlDelegate()
        {
            C::AutoescState<UrlEscaperContext> t___5399 = new C::AutoescState<UrlEscaperContext>(new UrlEscaperContext(0), null);
            this._state__677 = t___5399;
            this._subsidiary__678 = null;
        }
        public string Process(string ? known___5758)
        {
            return C::IContextDelegate<UrlEscaperContext>.ProcessDefault(this, known___5758);
        }
    }
}
