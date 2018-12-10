using System;

namespace RuAnnoy
{
    public abstract class DisposeBase : IDisposable
    {
        private bool isDisposed;

        ~DisposeBase()
        {
            this.Dispose(false);
        }

        public void Dispose()
        {
            this.Dispose(true);
            GC.SuppressFinalize(this);
        }

        protected virtual void Dispose(bool disposing)
        {
            if (this.isDisposed)
            {
                return;
            }

            if (disposing)
            {
                this.DisposeResources();
            }

            this.isDisposed = true;
        }

        protected abstract void DisposeResources();
    }
}
