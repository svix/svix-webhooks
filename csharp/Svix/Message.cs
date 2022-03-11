using System;
using Svix.Abstractions;
using Svix.Api;

namespace Svix
{
    public sealed class Message : SvixResourceBase, IMessage
    {
        private readonly IMessageApi _messageApi;

        public Message(ISvixClient svixClient, IMessageApi messageApi) 
            : base(svixClient)
        {
            _messageApi = messageApi ?? throw new ArgumentException(nameof(messageApi));
        }

        public void Create()
        {
            throw new System.NotImplementedException();
        }

        public void CreateAsync()
        {
            throw new System.NotImplementedException();
        }

        public void Get()
        {
            throw new System.NotImplementedException();
        }

        public void GetAsync()
        {
            throw new System.NotImplementedException();
        }

        public void List()
        {
            throw new System.NotImplementedException();
        }

        public void ListAsync()
        {
            throw new System.NotImplementedException();
        }
    }
}