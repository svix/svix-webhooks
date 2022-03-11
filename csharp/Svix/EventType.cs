using System;
using Svix.Abstractions;
using Svix.Api;

namespace Svix
{
    public sealed class EventType : SvixResourceBase, IEventType
    {
        private readonly IEventTypeApi _eventTypeApi;
        
        public EventType(ISvixClient svixClient, IEventTypeApi eventTypeApi) 
            : base(svixClient)
        {
            _eventTypeApi = eventTypeApi ?? throw new ArgumentNullException(nameof(eventTypeApi));
        }

        public void Archive()
        {
            throw new NotImplementedException();
        }
        
        public void ArchiveAsync()
        {
            throw new NotImplementedException();
        }

        public void Create()
        {
            throw new NotImplementedException();
        }
        
        public void CreateAsync()
        {
            throw new NotImplementedException();
        }

        public void Get()
        {
            throw new NotImplementedException();
        }
        
        public void GetAsync()
        {
            throw new NotImplementedException();
        }

        public void List()
        {
            throw new NotImplementedException();
        }
        
        public void ListAsync()
        {
            throw new NotImplementedException();
        }

        public void Update()
        {
            throw new NotImplementedException();
        }
        
        public void UpdateAsync()
        {
            throw new NotImplementedException();
        }
    }
}