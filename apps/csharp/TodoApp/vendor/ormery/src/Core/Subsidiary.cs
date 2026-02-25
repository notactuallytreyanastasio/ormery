namespace Ormery.Core
{
    public class Subsidiary
    {
        readonly IDelegate delegate__189;
        readonly ICodec codec__190;
        public Subsidiary(IDelegate delegate__192, ICodec codec__193)
        {
            this.delegate__189 = delegate__192;
            this.codec__190 = codec__193;
        }
        public IDelegate Delegate
        {
            get
            {
                return this.delegate__189;
            }
        }
        public ICodec Codec
        {
            get
            {
                return this.codec__190;
            }
        }
    }
}
